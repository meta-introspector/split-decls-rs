macro_rules! deps {
    () => {
        ForksafeTempfile!();
        TempfileOrTemppath!();
        AutoRemove!();
        Closed!();
        Mode!();
        Writable!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl ForksafeTempfile { pub fn new (tempfile : NamedTempFile , cleanup : AutoRemove , mode : handle :: Mode) -> Self { use handle :: Mode :: * ; ForksafeTempfile { inner : match mode { Closed => TempfileOrTemppath :: Temppath (tempfile . into_temp_path ()) , Writable => TempfileOrTemppath :: Tempfile (tempfile) , } , cleanup , owning_process_id : std :: process :: id () , } } }
    };
}

impl_8!();