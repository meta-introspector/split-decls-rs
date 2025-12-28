macro_rules! deps {
    () => {
        ArArchiveBuilder!();
        ArchiveBuilderBuilder!();
        ArArchiveBuilderBuilder!();
        ArchiveBuilder!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl ArchiveBuilderBuilder for ArArchiveBuilderBuilder { fn new_archive_builder < 'a > (& self , sess : & 'a Session) -> Box < dyn ArchiveBuilder + 'a > { Box :: new (ArArchiveBuilder :: new (sess , & DEFAULT_OBJECT_READER)) } }
    };
}

impl_28!();