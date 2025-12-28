macro_rules! deps {
    () => {
        AnnotationLocation!();
    };
}

macro_rules! AnnotationConfig {
    () => {
        deps!();
        pub struct AnnotationConfig < 'a > { pub binary_target : bool , pub annotate_runnables : bool , pub annotate_impls : bool , pub annotate_references : bool , pub annotate_method_references : bool , pub annotate_enum_variant_references : bool , pub location : AnnotationLocation , pub filter_adjacent_derive_implementations : bool , pub minicore : MiniCore < 'a > , }
    };
}

AnnotationConfig!();