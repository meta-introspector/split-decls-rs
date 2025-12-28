macro_rules! deps {
    () => {
        BenchmarkId!();
        ReportLink!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < 'a > ReportLink < 'a > { fn group (output_directory : & Path , group_id : & 'a str) -> ReportLink < 'a > { let path = PathBuf :: from (make_filename_safe (group_id)) ; ReportLink { name : group_id , path : if_exists (output_directory , & path) , } } fn function (output_directory : & Path , group_id : & str , function_id : & 'a str) -> ReportLink < 'a > { let mut path = PathBuf :: from (make_filename_safe (group_id)) ; path . push (make_filename_safe (function_id)) ; ReportLink { name : function_id , path : if_exists (output_directory , & path) , } } fn value (output_directory : & Path , group_id : & str , value_str : & 'a str) -> ReportLink < 'a > { let mut path = PathBuf :: from (make_filename_safe (group_id)) ; path . push (make_filename_safe (value_str)) ; ReportLink { name : value_str , path : if_exists (output_directory , & path) , } } fn individual (output_directory : & Path , id : & 'a BenchmarkId) -> ReportLink < 'a > { let path = PathBuf :: from (id . as_directory_name ()) ; ReportLink { name : id . as_title () , path : if_exists (output_directory , & path) , } } }
    };
}

impl_139!()