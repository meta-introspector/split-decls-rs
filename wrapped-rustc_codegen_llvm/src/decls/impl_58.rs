macro_rules! deps {
    () => {
        LlvmArchiveBuilderBuilder!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl ArchiveBuilderBuilder for LlvmArchiveBuilderBuilder { fn new_archive_builder < 'a > (& self , sess : & 'a Session) -> Box < dyn ArchiveBuilder + 'a > { Box :: new (ArArchiveBuilder :: new (sess , & LLVM_OBJECT_READER)) } }
    };
}

impl_58!()