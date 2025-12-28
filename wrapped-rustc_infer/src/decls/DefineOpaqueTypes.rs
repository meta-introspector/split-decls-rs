macro_rules! DefineOpaqueTypes {
    () => {
        # [doc = " Whether we should define opaque types or just treat them opaquely."] # [doc = ""] # [doc = " Currently only used to prevent predicate matching from matching anything"] # [doc = " against opaque types."] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum DefineOpaqueTypes { Yes , No , }
    };
}

DefineOpaqueTypes!()