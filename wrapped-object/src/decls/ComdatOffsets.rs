macro_rules! deps {
    () => {
        StringId!();
    };
}

macro_rules! ComdatOffsets {
    () => {
        deps!();
        # [derive (Clone , Copy)] struct ComdatOffsets { offset : usize , str_id : StringId , }
    };
}

ComdatOffsets!();