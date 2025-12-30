// Generated macro for impl_496 (impl)
macro_rules! Depcrate_objects_jobject_arrayimpl_496 {
() => {
// Module: crate::objects::jobject_array
// Provides: {"impl_496"}
// Dependencies: {}
unsafe impl < 'local , E : Reference + 'local > Reference for JObjectArray < 'local , E > { type Kind < 'env > = JObjectArray < 'env , E :: Kind < 'env > > where < E as Reference > :: Kind < 'env > : 'env ; type GlobalKind = JObjectArray < 'static , E :: GlobalKind > ; fn as_raw (& self) -> jobject { self . array . as_raw () } fn class_name () -> Cow < 'static , JNIStr > { let inner = E :: class_name () ; let inner = inner . to_str () ; let name = if inner . len () == 1 || inner . starts_with ("[") { format ! ("[{inner}") } else { format ! ("[L{inner};") } ; let name : JNIString = name . into () ; Cow :: Owned (name) } fn lookup_class < 'caller > (env : & Env < '_ > , loader_context : LoaderContext ,) -> crate :: errors :: Result < impl Deref < Target = Global < JClass < 'static > > > + 'caller > { let api = JObjectArrayAPI :: < E :: GlobalKind > :: get (env , & loader_context) ? ; Ok (& api . class) } unsafe fn kind_from_raw < 'env > (local_ref : jobject) -> Self :: Kind < 'env > { JObjectArray { array : JObject :: kind_from_raw (local_ref) , _marker : std :: marker :: PhantomData , } } unsafe fn global_kind_from_raw (global_ref : jobject) -> Self :: GlobalKind { JObjectArray { array : JObject :: global_kind_from_raw (global_ref) , _marker : std :: marker :: PhantomData , } } }
};
}
