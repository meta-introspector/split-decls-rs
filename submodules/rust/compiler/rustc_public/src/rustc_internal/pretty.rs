mkuse!{use std :: io ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use super :: run ;}

macro_rules! write_smir_pretty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_smir_pretty in module {}", module_path!());
    };
}

mkfn!{
    write_smir_pretty_introspect!();
    pub fn write_smir_pretty < 'tcx , W : io :: Write > (tcx : TyCtxt < 'tcx > , w : & mut W) -> io :: Result < () > { writeln ! (w , "// WARNING: This is highly experimental output it's intended for rustc_public developers only.") ? ; writeln ! (w , "// If you find a bug or want to improve the output open a issue at https://github.com/rust-lang/project-stable-mir.") ? ; let _ = run (tcx , | | { let items = crate :: all_local_items () ; let _ = items . iter () . map (| item | -> io :: Result < () > { item . emit_mir (w) }) . collect :: < Vec < _ > > () ; }) ; Ok (()) }
}