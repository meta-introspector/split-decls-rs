macro_rules! InspectFolder {
    () => {
        struct InspectFolder < 'f , C , F > { base : C , inspect_op : & 'f F , }
    };
}

InspectFolder!();