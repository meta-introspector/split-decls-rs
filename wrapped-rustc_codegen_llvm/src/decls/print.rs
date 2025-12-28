macro_rules! print {
    () => {
        pub (crate) fn print (req : & PrintRequest , out : & mut String , sess : & Session) { require_inited () ; let tm = create_informational_target_machine (sess , false) ; match req . kind { PrintKind :: TargetCPUs => print_target_cpus (sess , tm . raw () , out) , PrintKind :: TargetFeatures => print_target_features (sess , tm . raw () , out) , _ => bug ! ("rustc_codegen_llvm can't handle print request: {:?}" , req) , } }
    };
}

print!();