macro_rules! deps {
    () => {
        InDomainFn!();
        TransformCallback!();
        TransformFn!();
        CStrLike!();
        SliceTransform!();
    };
}

macro_rules! impl_382 {
    () => {
        deps!();
        impl SliceTransform { pub fn create (name : impl CStrLike , transform_fn : TransformFn , in_domain_fn : Option < InDomainFn > ,) -> SliceTransform { let cb = Box :: into_raw (Box :: new (TransformCallback { name : name . into_c_string () . unwrap () , transform_fn , in_domain_fn , })) ; let st = unsafe { ffi :: rocksdb_slicetransform_create (cb as * mut c_void , Some (slice_transform_destructor_callback) , Some (transform_callback) , Some (in_domain_callback) , None , Some (slice_transform_name_callback) ,) } ; SliceTransform { inner : st } } pub fn create_fixed_prefix (len : size_t) -> SliceTransform { SliceTransform { inner : unsafe { ffi :: rocksdb_slicetransform_create_fixed_prefix (len) } , } } pub fn create_noop () -> SliceTransform { SliceTransform { inner : unsafe { ffi :: rocksdb_slicetransform_create_noop () } , } } }
    };
}

impl_382!();