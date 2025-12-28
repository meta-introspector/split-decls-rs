macro_rules! deps {
    () => {
        File!();
        Position!();
    };
}

macro_rules! LookupByIdResult {
    () => {
        deps!();
        # [derive (Clone)] struct LookupByIdResult < 'a > { pub file : & 'a File , pub graph_pos : Position , pub file_pos : file :: Position , }
    };
}

LookupByIdResult!()