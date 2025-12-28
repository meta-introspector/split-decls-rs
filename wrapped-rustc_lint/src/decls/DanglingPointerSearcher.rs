macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! DanglingPointerSearcher {
    () => {
        deps!();
        # [doc = " This produces a dangling pointer:"] # [doc = " ```ignore (example)"] # [doc = " let ptr = CString::new(\"hello\").unwrap().as_ptr();"] # [doc = " foo(ptr)"] # [doc = " ```"] # [doc = ""] # [doc = " But this does not:"] # [doc = " ```ignore (example)"] # [doc = " foo(CString::new(\"hello\").unwrap().as_ptr())"] # [doc = " ```"] # [doc = ""] # [doc = " But this does:"] # [doc = " ```ignore (example)"] # [doc = " foo({ let ptr = CString::new(\"hello\").unwrap().as_ptr(); ptr })"] # [doc = " ```"] # [doc = ""] # [doc = " So we have to keep track of when we are inside of a function/method call argument."] struct DanglingPointerSearcher < 'lcx , 'tcx > { cx : & 'lcx LateContext < 'tcx > , # [doc = " Keeps track of whether we are inside of function/method call arguments,"] # [doc = " where this lint should not be emitted."] # [doc = ""] # [doc = " See [the main doc][`Self`] for examples."] inside_call_args : bool , }
    };
}

DanglingPointerSearcher!();