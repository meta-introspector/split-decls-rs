macro_rules! TrimmedPatch {
    () => {
        # [derive (Clone , Debug)] pub (crate) struct TrimmedPatch < 'a > { pub (crate) original_span : Range < usize > , pub (crate) span : Range < usize > , pub (crate) replacement : Cow < 'a , str > , }
    };
}

TrimmedPatch!();