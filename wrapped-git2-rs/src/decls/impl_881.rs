macro_rules! impl_881 {
    () => {
        impl RepositoryInitMode { is_bit_set ! (is_shared_umask , RepositoryInitMode :: SHARED_UMASK) ; is_bit_set ! (is_shared_group , RepositoryInitMode :: SHARED_GROUP) ; is_bit_set ! (is_shared_all , RepositoryInitMode :: SHARED_ALL) ; }
    };
}

impl_881!()