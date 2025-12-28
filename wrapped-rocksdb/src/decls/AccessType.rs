macro_rules! AccessType {
    () => {
        enum AccessType < 'a > { ReadWrite , ReadOnly { error_if_log_file_exist : bool } , Secondary { secondary_path : & 'a Path } , WithTTL { ttl : Duration } , }
    };
}

AccessType!()