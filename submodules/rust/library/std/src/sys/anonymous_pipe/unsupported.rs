mkuse!{use crate :: io ;}
mkuse!{pub use crate :: sys :: pipe :: AnonPipe ;}

macro_rules! pipe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pipe in module {}", module_path!());
    };
}

mkfn!{
    pipe_introspect!();
    # [inline] pub fn pipe () -> io :: Result < (AnonPipe , AnonPipe) > { Err (io :: Error :: UNSUPPORTED_PLATFORM) }
}