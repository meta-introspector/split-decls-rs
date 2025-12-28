macro_rules! deps {
    () => {
        Id!();
        Reference!();
        Head!();
        Kind!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        # [doc = " Access"] impl < 'repo > Head < 'repo > { # [doc = " Returns the name of this references, always `HEAD`."] pub fn name (& self) -> & 'static FullNameRef { "HEAD" . try_into () . expect ("HEAD is valid") } # [doc = " Returns the full reference name of this head if it is not detached, or `None` otherwise."] pub fn referent_name (& self) -> Option < & FullNameRef > { Some (match & self . kind { Kind :: Symbolic (r) => r . name . as_ref () , Kind :: Unborn (name) => name . as_ref () , Kind :: Detached { .. } => return None , }) } # [doc = " Returns true if this instance is detached, and points to an object directly."] pub fn is_detached (& self) -> bool { matches ! (self . kind , Kind :: Detached { .. }) } # [doc = " Returns true if this instance is not yet born, hence it points to a ref that doesn't exist yet."] # [doc = ""] # [doc = " This is the case in a newly initialized repository."] pub fn is_unborn (& self) -> bool { matches ! (self . kind , Kind :: Unborn (_)) } # [doc = " Returns the id the head points to, which isn't possible on unborn heads."] pub fn id (& self) -> Option < crate :: Id < 'repo > > { match & self . kind { Kind :: Symbolic (r) => r . target . try_id () . map (| oid | oid . to_owned () . attach (self . repo)) , Kind :: Detached { peeled , target } => { (* peeled) . unwrap_or_else (| | target . to_owned ()) . attach (self . repo) . into () } Kind :: Unborn (_) => None , } } # [doc = " Try to transform this instance into the symbolic reference that it points to, or return `None` if head is detached or unborn."] pub fn try_into_referent (self) -> Option < crate :: Reference < 'repo > > { match self . kind { Kind :: Symbolic (r) => r . attach (self . repo) . into () , _ => None , } } }
    };
}

impl_129!();