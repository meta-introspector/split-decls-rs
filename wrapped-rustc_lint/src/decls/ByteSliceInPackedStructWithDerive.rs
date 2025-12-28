macro_rules! ByteSliceInPackedStructWithDerive {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_byte_slice_in_packed_struct_with_derive)] # [help] pub (crate) struct ByteSliceInPackedStructWithDerive { pub ty : String , }
    };
}

ByteSliceInPackedStructWithDerive!()