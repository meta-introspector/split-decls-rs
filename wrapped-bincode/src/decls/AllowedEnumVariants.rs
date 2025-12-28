macro_rules! AllowedEnumVariants {
    () => {
        # [doc = " Indicates which enum variants are allowed"] # [non_exhaustive] # [derive (Debug , PartialEq , Eq)] pub enum AllowedEnumVariants { # [doc = " All values between `min` and `max` (inclusive) are allowed"] # [allow (missing_docs)] Range { min : u32 , max : u32 } , # [doc = " Each one of these values is allowed"] Allowed (& 'static [u32]) , }
    };
}

AllowedEnumVariants!();