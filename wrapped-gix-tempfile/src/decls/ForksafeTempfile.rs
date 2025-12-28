macro_rules! deps {
    () => {
        TempfileOrTemppath!();
        AutoRemove!();
    };
}

macro_rules! ForksafeTempfile {
    () => {
        deps!();
        pub (crate) struct ForksafeTempfile { inner : TempfileOrTemppath , cleanup : AutoRemove , pub owning_process_id : u32 , }
    };
}

ForksafeTempfile!();