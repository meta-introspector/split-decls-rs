// Generated macro for impl_64 (impl)
macro_rules! Depcrate_descriptors_descimpl_64 {
() => {
// Module: crate::descriptors::desc
// Provides: {"impl_64"}
// Dependencies: {}
unsafe impl < 'local , 'obj_ref , T > Desc < 'local , T > for & 'obj_ref Global < T > where T : Reference + AsRef < T > + AsRef < JObject < 'static > > + Into < JObject < 'static > > + Default + Send + Sync + 'static , { type Output = & 'obj_ref T ; fn lookup (self , _ : & mut Env < 'local >) -> Result < Self :: Output > { Ok (self . as_ref ()) } }
};
}
