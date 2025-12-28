macro_rules! deps {
    () => {
        Action!();
        Context!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [doc = " Access"] impl Action { # [doc = " Return the payload of store or erase actions."] pub fn payload (& self) -> Option < & BStr > { use bstr :: ByteSlice ; match self { Action :: Get (_) => None , Action :: Store (p) | Action :: Erase (p) => Some (p . as_bstr ()) , } } # [doc = " Return the context of a get operation, or `None`."] # [doc = ""] # [doc = " The opposite of [`payload`][Action::payload()]."] pub fn context (& self) -> Option < & Context > { match self { Action :: Get (ctx) => Some (ctx) , Action :: Erase (_) | Action :: Store (_) => None , } } # [doc = " Return the mutable context of a get operation, or `None`."] pub fn context_mut (& mut self) -> Option < & mut Context > { match self { Action :: Get (ctx) => Some (ctx) , Action :: Erase (_) | Action :: Store (_) => None , } } # [doc = " Returns true if this action expects output from the helper."] pub fn expects_output (& self) -> bool { matches ! (self , Action :: Get (_)) } # [doc = " The name of the argument to describe this action. If `is_external` is true, the target program is"] # [doc = " a custom credentials helper, not a built-in one."] pub fn as_arg (& self , is_external : bool) -> & str { match self { Action :: Get (_) if is_external => "get" , Action :: Get (_) => "fill" , Action :: Store (_) if is_external => "store" , Action :: Store (_) => "approve" , Action :: Erase (_) if is_external => "erase" , Action :: Erase (_) => "reject" , } } }
    };
}

impl_8!()