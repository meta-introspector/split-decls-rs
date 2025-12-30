// Generated macro for TransformExt (trait)
macro_rules! Depcrate_extTransformExt {
() => {
// Module: crate::ext
// Provides: {"TransformExt"}
// Dependencies: {}
# [doc = " An extension trait for [`Transform`]s that provides a variety of convenient adapters."] pub trait TransformExt < S , Req > : Transform < S , Req > { # [doc = " Return a new `Transform` whose init error is mapped to to a different type."] fn map_init_err < F , E > (self , f : F) -> TransformMapInitErr < Self , S , Req , F , E > where Self : Sized , F : Fn (Self :: InitError) -> E + Clone , { TransformMapInitErr :: new (self , f) } }
};
}
