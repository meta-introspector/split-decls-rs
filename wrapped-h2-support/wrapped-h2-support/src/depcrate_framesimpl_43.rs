// Generated macro for impl_43 (impl)
macro_rules! Depcrate_framesimpl_43 {
() => {
// Module: crate::frames
// Provides: {"impl_43"}
// Dependencies: {}
impl Mock < frame :: PushPromise > { pub fn request < M , U > (self , method : M , uri : U) -> Self where M : TryInto < http :: Method > , M :: Error : fmt :: Debug , U : TryInto < http :: Uri > , U :: Error : fmt :: Debug , { let method = method . try_into () . unwrap () ; let uri = uri . try_into () . unwrap () ; let (id , promised , _ , fields) = self . into_parts () ; let extensions = Default :: default () ; let pseudo = frame :: Pseudo :: request (method , uri , extensions) ; let frame = frame :: PushPromise :: new (id , promised , pseudo , fields) ; Mock (frame) } pub fn fields (self , fields : HeaderMap) -> Self { let (id , promised , pseudo , _) = self . into_parts () ; let frame = frame :: PushPromise :: new (id , promised , pseudo , fields) ; Mock (frame) } pub fn field < K , V > (self , key : K , value : V) -> Self where K : TryInto < http :: header :: HeaderName > , K :: Error : fmt :: Debug , V : TryInto < http :: header :: HeaderValue > , V :: Error : fmt :: Debug , { let (id , promised , pseudo , mut fields) = self . into_parts () ; fields . insert (key . try_into () . unwrap () , value . try_into () . unwrap ()) ; let frame = frame :: PushPromise :: new (id , promised , pseudo , fields) ; Mock (frame) } fn into_parts (self) -> (StreamId , StreamId , frame :: Pseudo , HeaderMap) { assert ! (self . 0 . is_end_headers () , "unset eoh will be lost") ; let id = self . 0 . stream_id () ; let promised = self . 0 . promised_id () ; let parts = self . 0 . into_parts () ; (id , promised , parts . 0 , parts . 1) } }
};
}
