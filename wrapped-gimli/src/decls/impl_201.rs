macro_rules! deps {
    () => {
        Result!();
        Augmentation!();
        Reader!();
        PointerEncodingParameters!();
        AugmentationData!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl AugmentationData { fn parse < R : Reader > (augmentation : & Augmentation , encoding_parameters : & PointerEncodingParameters < '_ , R > , input : & mut R ,) -> Result < AugmentationData > { let aug_data_len = input . read_uleb128 () . and_then (R :: Offset :: from_u64) ? ; let rest = & mut input . split (aug_data_len) ? ; let mut augmentation_data = AugmentationData :: default () ; if let Some (encoding) = augmentation . lsda { let lsda = parse_encoded_pointer (encoding , encoding_parameters , rest) ? ; augmentation_data . lsda = Some (lsda) ; } Ok (augmentation_data) } }
    };
}

impl_201!();