macro_rules! deps {
    () => {
        MemoryLayoutHoverRenderKind!();
    };
}

macro_rules! MemoryLayoutHoverConfig {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct MemoryLayoutHoverConfig { pub size : Option < MemoryLayoutHoverRenderKind > , pub offset : Option < MemoryLayoutHoverRenderKind > , pub alignment : Option < MemoryLayoutHoverRenderKind > , pub padding : Option < MemoryLayoutHoverRenderKind > , pub niches : bool , }
    };
}

MemoryLayoutHoverConfig!();