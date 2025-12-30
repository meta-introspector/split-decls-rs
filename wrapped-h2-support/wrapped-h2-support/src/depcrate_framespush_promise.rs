// Generated macro for push_promise (function)
macro_rules! Depcrate_framespush_promise {
() => {
// Module: crate::frames
// Provides: {"push_promise"}
// Dependencies: {}
pub fn push_promise < T1 , T2 > (id : T1 , promised : T2) -> Mock < frame :: PushPromise > where T1 : Into < StreamId > , T2 : Into < StreamId > , { Mock (frame :: PushPromise :: new (id . into () , promised . into () , frame :: Pseudo :: default () , HeaderMap :: default () ,)) }
};
}
