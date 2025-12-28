macro_rules! filter_targets {
    () => {
        fn filter_targets (allowed_targets : & mut Vec < Target > , target_group : & 'static [Target] , target_group_name : & 'static str , target : Target , added_fake_targets : & mut Vec < & 'static str > ,) { if target_group . contains (& target) { return ; } if allowed_targets . iter () . filter (| at | target_group . contains (at)) . count () < 2 { return ; } allowed_targets . retain (| t | ! target_group . contains (t)) ; added_fake_targets . push (target_group_name) ; }
    };
}

filter_targets!()