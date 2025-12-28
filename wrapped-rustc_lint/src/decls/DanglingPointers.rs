macro_rules! DanglingPointers {
    () => {
        # [doc = " FIXME: false negatives (i.e. the lint is not emitted when it should be)"] # [doc = " 1. Ways to get a temporary that are not recognized:"] # [doc = "    - `owning_temporary.field`"] # [doc = "    - `owning_temporary[index]`"] # [doc = " 2. No checks for ref-to-ptr conversions:"] # [doc = "    - `&raw [mut] temporary`"] # [doc = "    - `&temporary as *(const|mut) _`"] # [doc = "    - `ptr::from_ref(&temporary)` and friends"] # [derive (Clone , Copy , Default)] pub (crate) struct DanglingPointers ;
    };
}

DanglingPointers!();