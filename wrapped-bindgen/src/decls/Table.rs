macro_rules! deps {
    () => {
        Column!();
    };
}

macro_rules! Table {
    () => {
        deps!();
        # [derive (Default)] struct Table { offset : usize , len : usize , width : usize , columns : [Column ; 6] , }
    };
}

Table!();