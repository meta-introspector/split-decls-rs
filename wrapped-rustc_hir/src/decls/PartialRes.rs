macro_rules! deps {
    () => {
        Res!();
    };
}

macro_rules! PartialRes {
    () => {
        deps!();
        # [doc = " The result of resolving a path before lowering to HIR,"] # [doc = " with \"module\" segments resolved and associated item"] # [doc = " segments deferred to type checking."] # [doc = " `base_res` is the resolution of the resolved part of the"] # [doc = " path, `unresolved_segments` is the number of unresolved"] # [doc = " segments."] # [doc = ""] # [doc = " ```text"] # [doc = " module::Type::AssocX::AssocY::MethodOrAssocType"] # [doc = " ^~~~~~~~~~~~  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"] # [doc = " base_res      unresolved_segments = 3"] # [doc = ""] # [doc = " <T as Trait>::AssocX::AssocY::MethodOrAssocType"] # [doc = "       ^~~~~~~~~~~~~~  ^~~~~~~~~~~~~~~~~~~~~~~~~"] # [doc = "       base_res        unresolved_segments = 2"] # [doc = " ```"] # [derive (Copy , Clone , Debug)] pub struct PartialRes { base_res : Res < NodeId > , unresolved_segments : usize , }
    };
}

PartialRes!();