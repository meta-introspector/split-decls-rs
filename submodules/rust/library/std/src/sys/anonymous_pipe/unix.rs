mkuse!{use crate :: io ;}
mkuse!{use crate :: sys :: fd :: FileDesc ;}
mkuse!{use crate :: sys :: pipe :: anon_pipe ;}
mkuse!{use crate :: sys_common :: IntoInner ;}
mkitem!{pub type AnonPipe = FileDesc ;}

macro_rules! pipe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pipe in module {}", module_path!());
    };
}

mkfn!{
    pipe_introspect!();
    # [inline] pub fn pipe () -> io :: Result < (AnonPipe , AnonPipe) > { anon_pipe () . map (| (rx , wx) | (rx . into_inner () , wx . into_inner ())) }
}