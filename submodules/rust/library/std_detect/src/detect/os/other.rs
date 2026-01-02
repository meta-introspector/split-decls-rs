mkuse!{use crate :: detect :: cache ;}

macro_rules! detect_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function detect_features in module {}", module_path!());
    };
}

mkfn!{
    detect_features_introspect!();
    # [allow (dead_code)] pub (crate) fn detect_features () -> cache :: Initializer { cache :: Initializer :: default () }
}