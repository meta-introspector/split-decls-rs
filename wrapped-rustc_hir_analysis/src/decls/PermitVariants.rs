macro_rules! PermitVariants {
    () => {
        # [doc = " Whether to permit a path to resolve to an enum variant."] # [derive (Debug , Clone , Copy)] pub enum PermitVariants { Yes , No , }
    };
}

PermitVariants!();