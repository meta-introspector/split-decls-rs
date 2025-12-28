macro_rules! align_down {
    () => {
        # [inline] fn align_down (sp : * mut usize) -> * mut usize { let sp = (sp as usize) & ! (16 - 1) ; sp as * mut usize }
    };
}

align_down!()