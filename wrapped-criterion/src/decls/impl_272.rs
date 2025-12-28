macro_rules! deps {
    () => {
        BenchmarkId!();
        ReportContext!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl ReportContext { pub fn report_path < P : AsRef < Path > + ? Sized > (& self , id : & BenchmarkId , file_name : & P) -> PathBuf { let mut path = self . output_directory . clone () ; path . push (id . as_directory_name ()) ; path . push ("report") ; path . push (file_name) ; path } }
    };
}

impl_272!();