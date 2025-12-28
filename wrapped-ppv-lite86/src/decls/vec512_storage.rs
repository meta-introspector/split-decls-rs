macro_rules! vec512_storage {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq , Default)] pub struct vec512_storage { v128 : [vec128_storage ; 4] , }
    };
}

vec512_storage!();