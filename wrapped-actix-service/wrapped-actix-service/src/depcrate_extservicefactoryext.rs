// Generated macro for ServiceFactoryExt (trait)
macro_rules! Depcrate_extServiceFactoryExt {
() => {
// Module: crate::ext
// Provides: {"ServiceFactoryExt"}
// Dependencies: {}
# [doc = " An extension trait for [`ServiceFactory`]s that provides a variety of convenient adapters."] pub trait ServiceFactoryExt < Req > : ServiceFactory < Req > { # [doc = " Map this service's output to a different type, returning a new service"] # [doc = " of the resulting type."] fn map < F , R > (self , f : F) -> crate :: map :: MapServiceFactory < Self , F , Req , R > where Self : Sized , F : FnMut (Self :: Response) -> R + Clone , { crate :: map :: MapServiceFactory :: new (self , f) } # [doc = " Map this service's error to a different error, returning a new service."] fn map_err < F , E > (self , f : F) -> crate :: map_err :: MapErrServiceFactory < Self , Req , F , E > where Self : Sized , F : Fn (Self :: Error) -> E + Clone , { crate :: map_err :: MapErrServiceFactory :: new (self , f) } # [doc = " Map this factory's init error to a different error, returning a new service."] fn map_init_err < F , E > (self , f : F) -> crate :: map_init_err :: MapInitErr < Self , F , Req , E > where Self : Sized , F : Fn (Self :: InitError) -> E + Clone , { crate :: map_init_err :: MapInitErr :: new (self , f) } # [doc = " Call another service after call to this one has resolved successfully."] fn and_then < I , SF1 > (self , factory : I) -> AndThenServiceFactory < Self , SF1 , Req > where Self : Sized , Self :: Config : Clone , I : IntoServiceFactory < SF1 , Self :: Response > , SF1 : ServiceFactory < Self :: Response , Config = Self :: Config , Error = Self :: Error , InitError = Self :: InitError , > , { AndThenServiceFactory :: new (self , factory . into_factory ()) } }
};
}
