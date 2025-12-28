macro_rules! MaskProperty {
    () => {
        # [doc = " This type can represent any Unicode mask property."] # [doc = ""] # [doc = " This is intended to be used in situations where the exact unicode property needed is"] # [doc = " only known at runtime, for example in regex engines."] # [doc = ""] # [doc = " The values are intended to be identical to ICU4C's UProperty enum"] # [non_exhaustive] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [allow (dead_code)] # [allow (missing_docs)] enum MaskProperty { GeneralCategoryMask = 0x2000 , }
    };
}

MaskProperty!()