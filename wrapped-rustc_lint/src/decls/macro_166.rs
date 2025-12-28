macro_rules! macro_166 {
    () => {
        declare_lint_pass ! (DropForgetUseless => [DROPPING_REFERENCES , FORGETTING_REFERENCES , DROPPING_COPY_TYPES , FORGETTING_COPY_TYPES , UNDROPPED_MANUALLY_DROPS]) ;
    };
}

macro_166!()