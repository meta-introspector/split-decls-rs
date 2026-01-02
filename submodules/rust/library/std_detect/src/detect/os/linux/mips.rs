mkuse!{use super :: auxvec ;}
mkuse!{use crate :: detect :: { Feature , bit , cache } ;}

macro_rules! detect_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function detect_features in module {}", module_path!());
    };
}

mkfn!{
    detect_features_introspect!();
    # [doc = " Try to read the features from the auxiliary vector."] pub (crate) fn detect_features () -> cache :: Initializer { let mut value = cache :: Initializer :: default () ; let enable_feature = | value : & mut cache :: Initializer , f , enable | { if enable { value . set (f as u32) ; } } ; if let Ok (auxv) = auxvec :: auxv () { enable_feature (& mut value , Feature :: msa , bit :: test (auxv . hwcap , 1)) ; return value ; } value }
}