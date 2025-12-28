macro_rules! macro_32 {
    () => {
        bitflags ! { # [doc = " How to handle reference updates."] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct RemoteUpdateFlags : u32 { # [doc = " Write the fetch results to FETCH_HEAD."] const UPDATE_FETCHHEAD = raw :: GIT_REMOTE_UPDATE_FETCHHEAD as u32 ; # [doc = " Report unchanged tips in the update_tips callback."] const REPORT_UNCHANGED = raw :: GIT_REMOTE_UPDATE_REPORT_UNCHANGED as u32 ; } }
    };
}

macro_32!()