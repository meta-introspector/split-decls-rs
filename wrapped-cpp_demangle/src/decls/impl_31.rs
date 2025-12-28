macro_rules! deps {
    () => {
        ArgScopeStackExt!();
        ArgScope!();
        ArgScopeStack!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'prev , 'subs > ArgScopeStackExt < 'prev , 'subs > for Option < ArgScopeStack < 'prev , 'subs > > { fn push (& 'prev self , item : & 'subs dyn ArgScope < 'subs , 'subs > ,) -> Option < ArgScopeStack < 'prev , 'subs > > { log ! ("ArgScopeStack::push: {:?}" , item) ; Some (ArgScopeStack { prev : self . as_ref () , in_arg : None , item : item , }) } }
    };
}

impl_31!()