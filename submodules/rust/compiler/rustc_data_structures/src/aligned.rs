mkuse!{use std :: marker :: PointeeSized ;}
mkuse!{use std :: ptr :: Alignment ;}

macro_rules! align_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function align_of in module {}", module_path!());
    };
}

mkfn!{
    align_of_introspect!();
    # [doc = " Returns the ABI-required minimum alignment of a type in bytes."] # [doc = ""] # [doc = " This is equivalent to [`align_of`], but also works for some unsized"] # [doc = " types (e.g. slices or rustc's `List`s)."] pub const fn align_of < T : ? Sized + Aligned > () -> Alignment { T :: ALIGN }
}
mkitem!{mktrait!{# [doc = " A type with a statically known alignment."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `Self::ALIGN` must be equal to the alignment of `Self`. For sized types it"] # [doc = " is [`align_of::<Self>()`], for unsized types it depends on the type, for"] # [doc = " example `[T]` has alignment of `T`."] # [doc = ""] # [doc = " [`align_of::<Self>()`]: align_of"] pub unsafe trait Aligned : PointeeSized { # [doc = " Alignment of `Self`."] const ALIGN : Alignment ; }}}
mkitem!{mkimpl!{unsafe impl < T > Aligned for T { const ALIGN : Alignment = Alignment :: of :: < Self > () ; }}}
mkitem!{mkimpl!{unsafe impl < T > Aligned for [T] { const ALIGN : Alignment = Alignment :: of :: < T > () ; }}}