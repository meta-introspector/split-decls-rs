macro_rules! deps {
    () => {
        OwnedData!();
        SharedData!();
    };
}

macro_rules! Data {
    () => {
        deps!();
        # [doc = " Serialized database"] pub enum Data < 'conn > { # [doc = " Shared (SQLITE_SERIALIZE_NOCOPY) serialized database"] Shared (SharedData < 'conn >) , # [doc = " Owned serialized database"] Owned (OwnedData) , }
    };
}

Data!()