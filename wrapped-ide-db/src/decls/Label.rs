macro_rules! Label {
    () => {
        # [doc = " A type to specify UI label, like an entry in the list of assists. Enforces"] # [doc = " proper casing:"] # [doc = ""] # [doc = "    Frobnicate bar"] # [doc = ""] # [doc = " Note the upper-case first letter and the absence of `.` at the end."] # [derive (Clone)] pub struct Label (String) ;
    };
}

Label!()