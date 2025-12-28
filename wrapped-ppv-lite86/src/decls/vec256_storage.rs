macro_rules! vec256_storage {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq , Default)] pub struct vec256_storage { v128 : [vec128_storage ; 2] , }
    };
}

vec256_storage!();