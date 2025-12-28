macro_rules! deps {
    () => {
        ParDrainProducer!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < T : Send > UnindexedProducer for ParDrainProducer < T > { type Item = T ; # [cfg_attr (feature = "inline-more" , inline)] fn split (self) -> (Self , Option < Self >) { let (left , right) = self . iter . clone () . split () ; mem :: forget (self) ; let left = ParDrainProducer { iter : left } ; let right = right . map (| right | ParDrainProducer { iter : right }) ; (left , right) } # [cfg_attr (feature = "inline-more" , inline)] fn fold_with < F > (mut self , mut folder : F) -> F where F : Folder < Self :: Item > , { for item in & mut self . iter { folder = folder . consume (unsafe { item . read () }) ; if folder . full () { return folder ; } } mem :: forget (self) ; folder } }
    };
}

impl_176!();