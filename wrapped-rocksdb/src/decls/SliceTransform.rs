macro_rules! SliceTransform {
    () => {
        # [doc = " A `SliceTransform` is a generic pluggable way of transforming one string"] # [doc = " to another. Its primary use-case is in configuring rocksdb"] # [doc = " to store prefix blooms by setting prefix_extractor in"] # [doc = " ColumnFamilyOptions."] pub struct SliceTransform { pub inner : * mut ffi :: rocksdb_slicetransform_t , }
    };
}

SliceTransform!();