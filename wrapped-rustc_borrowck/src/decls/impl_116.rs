macro_rules! deps {
    () => {
        AnnotatedBorrowFnSignature!();
        MirBorrowckCtxt!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < 'tcx > AnnotatedBorrowFnSignature < 'tcx > { # [doc = " Annotate the provided diagnostic with information about borrow from the fn signature that"] # [doc = " helps explain."] pub (crate) fn emit (& self , cx : & MirBorrowckCtxt < '_ , '_ , 'tcx > , diag : & mut Diag < '_ >) -> String { match self { & AnnotatedBorrowFnSignature :: Closure { argument_ty , argument_span } => { diag . span_label (argument_span , format ! ("has type `{}`" , cx . get_name_for_ty (argument_ty , 0)) ,) ; cx . get_region_name_for_ty (argument_ty , 0) } & AnnotatedBorrowFnSignature :: AnonymousFunction { argument_ty , argument_span , return_ty , return_span , } => { let argument_ty_name = cx . get_name_for_ty (argument_ty , 0) ; diag . span_label (argument_span , format ! ("has type `{argument_ty_name}`")) ; let return_ty_name = cx . get_name_for_ty (return_ty , 0) ; let types_equal = return_ty_name == argument_ty_name ; diag . span_label (return_span , format ! ("{}has type `{}`" , if types_equal { "also " } else { "" } , return_ty_name ,) ,) ; diag . note ("argument and return type have the same lifetime due to lifetime elision rules" ,) ; diag . note ("to learn more, visit <https://doc.rust-lang.org/book/ch10-03-\
                     lifetime-syntax.html#lifetime-elision>" ,) ; cx . get_region_name_for_ty (return_ty , 0) } AnnotatedBorrowFnSignature :: NamedFunction { arguments , return_ty , return_span } => { let region_name = cx . get_region_name_for_ty (* return_ty , 0) ; for (_ , argument_span) in arguments { diag . span_label (* argument_span , format ! ("has lifetime `{region_name}`")) ; } diag . span_label (* return_span , format ! ("also has lifetime `{region_name}`" ,)) ; diag . help (format ! ("use data from the highlighted arguments which match the `{region_name}` lifetime of \
                     the return type" ,)) ; region_name } } } }
    };
}

impl_116!();