macro_rules! deps {
    () => {
        State!();
        CommitRefIter!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl < 'a > CommitRefIter < 'a > { # [doc = " Create a commit iterator from data."] pub fn from_bytes (data : & 'a [u8]) -> CommitRefIter < 'a > { CommitRefIter { data , state : State :: default () , } } }
    };
}

impl_31!();