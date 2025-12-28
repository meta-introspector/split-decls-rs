macro_rules! deps {
    () => {
        TermsContext!();
    };
}

macro_rules! terms {
    () => {
        deps!();
        # [doc = " Defines the `TermsContext` basically houses an arena where we can"] # [doc = " allocate terms."] mod terms ;
    };
}

terms!()