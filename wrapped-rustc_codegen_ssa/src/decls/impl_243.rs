macro_rules! deps {
    () => {
        SharedEmitterMessage!();
        SharedEmitterMain!();
        Diagnostic!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl SharedEmitterMain { fn check (& self , sess : & Session , blocking : bool) { loop { let message = if blocking { match self . receiver . recv () { Ok (message) => Ok (message) , Err (_) => Err (()) , } } else { match self . receiver . try_recv () { Ok (message) => Ok (message) , Err (_) => Err (()) , } } ; match message { Ok (SharedEmitterMessage :: Diagnostic (diag)) => { let dcx = sess . dcx () ; let mut d = rustc_errors :: DiagInner :: new_with_messages (diag . level , diag . messages) ; d . code = diag . code ; d . children = diag . children . into_iter () . map (| sub | rustc_errors :: Subdiag { level : sub . level , messages : sub . messages , span : MultiSpan :: new () , }) . collect () ; d . args = diag . args ; dcx . emit_diagnostic (d) ; sess . dcx () . abort_if_errors () ; } Ok (SharedEmitterMessage :: InlineAsmError (span , msg , level , source)) => { assert_matches ! (level , Level :: Error | Level :: Warning | Level :: Note) ; let mut err = Diag :: < () > :: new (sess . dcx () , level , msg) ; if ! span . is_dummy () { err . span (span . span ()) ; } if let Some ((buffer , spans)) = source { let source = sess . source_map () . new_source_file (FileName :: inline_asm_source_code (& buffer) , buffer) ; let spans : Vec < _ > = spans . iter () . map (| sp | { Span :: with_root_ctxt (source . normalized_byte_pos (sp . start as u32) , source . normalized_byte_pos (sp . end as u32) ,) }) . collect () ; err . span_note (spans , "instantiated into assembly here") ; } err . emit () ; } Ok (SharedEmitterMessage :: Fatal (msg)) => { sess . dcx () . fatal (msg) ; } Err (_) => { break ; } } } } }
    };
}

impl_243!();