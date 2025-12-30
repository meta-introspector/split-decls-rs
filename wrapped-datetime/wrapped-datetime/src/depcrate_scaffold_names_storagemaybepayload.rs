// Generated macro for MaybePayload (trait)
macro_rules! Depcrate_scaffold_names_storageMaybePayload {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"MaybePayload"}
// Dependencies: {}
# [doc = " A type that may or may not be a [`DataPayload`] and may or may not contain"] # [doc = " a value depending on the type parameter `Variables`."] # [doc = ""] # [doc = " Helper trait for [`DateTimeNamesMarker`]."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. Do not implement this trait in userland unless you are prepared for things to occasionally break."] # [doc = " </div>"] # [allow (missing_docs)] pub trait MaybePayload < M : DynamicDataMarker , Variables > : UnstableSealed { fn new_empty () -> Self ; fn load_put < P > (& mut self , provider : & P , req : DataRequest , variables : Variables ,) -> Result < Result < DataResponseMetadata , DataError > , MaybePayloadError > where P : BoundDataProvider < M > + ? Sized , Self : Sized ; fn get (& self) -> DataPayloadWithVariablesBorrowed < '_ , M , Variables > ; }
};
}
