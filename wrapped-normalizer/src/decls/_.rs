macro_rules! deps {
    () => {
        Baked!();
    };
}

macro_rules! _ {
    () => {
        deps!();
        # [cfg (feature = "compiled_data")] # [allow (unused_imports)] const _ : () = { use icu_normalizer_data :: * ; pub mod icu { pub use crate as normalizer ; pub use icu_collections as collections ; } make_provider ! (Baked) ; impl_normalizer_nfc_v1 ! (Baked) ; impl_normalizer_nfd_data_v1 ! (Baked) ; impl_normalizer_nfd_supplement_v1 ! (Baked) ; impl_normalizer_nfd_tables_v1 ! (Baked) ; impl_normalizer_nfkd_data_v1 ! (Baked) ; impl_normalizer_nfkd_tables_v1 ! (Baked) ; impl_normalizer_uts46_data_v1 ! (Baked) ; } ;
    };
}

_!()