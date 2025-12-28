macro_rules! deps {
    () => {
        Shallow!();
        Mapping!();
        Source!();
        Arguments!();
    };
}

macro_rules! add_wants {
    () => {
        deps!();
        # [doc = " Add all 'wants' to `arguments` once it's known negotiation is necessary."] # [doc = ""] # [doc = " This is a call to be made when [`mark_complete_and_common_ref()`] returned [`Action::MustNegotiate`]."] # [doc = " That variant also contains the `remote_ref_target_known` field which is supposed to be passed here."] # [doc = ""] # [doc = " `objects` are used to see if remote ids are known here and are tags, in which case they are also added as 'haves' as"] # [doc = " [negotiators](gix_negotiate::Negotiator) don't see tags at all."] # [doc = ""] # [doc = " * `ref_map` is the state of refs as known on the remote."] # [doc = " * `shallow` defines if the history should be shallow."] # [doc = " * `mapping_is_ignored` is typically initialized with [`make_refmapping_ignore_predicate`]."] # [doc = ""] # [doc = " Returns `true` if at least one [want](crate::fetch::Arguments::want()) was added, or `false` otherwise."] # [doc = " Note that not adding a single want can make the remote hang, so it's avoided on the client side by ending the fetch operation."] pub fn add_wants (objects : & impl gix_object :: FindHeader , arguments : & mut crate :: fetch :: Arguments , ref_map : & RefMap , remote_ref_target_known : & [bool] , shallow : & Shallow , mapping_is_ignored : impl Fn (& refmap :: Mapping) -> bool ,) -> bool { let is_shallow = ! matches ! (shallow , Shallow :: NoChange) ; let mut has_want = false ; let wants = ref_map . mappings . iter () . zip (remote_ref_target_known) . filter_map (| (m , known) | (is_shallow || ! * known) . then_some (m)) . filter (| m | ! mapping_is_ignored (m)) ; for want in wants { let id_on_remote = want . remote . as_id () ; if ! arguments . can_use_ref_in_want () || matches ! (want . remote , refmap :: Source :: ObjectId (_)) { if let Some (id) = id_on_remote { arguments . want (id) ; has_want = true ; } } else { arguments . want_ref (want . remote . as_name () . expect ("name available if this isn't an object id") ,) ; has_want = true ; } let id_is_annotated_tag_we_have = id_on_remote . and_then (| id | objects . try_header (id) . ok () . flatten () . map (| h | (id , h))) . filter (| (_ , h) | h . kind == gix_object :: Kind :: Tag) . map (| (id , _) | id) ; if let Some (tag_on_remote) = id_is_annotated_tag_we_have { arguments . have (tag_on_remote) ; } } has_want }
    };
}

add_wants!();