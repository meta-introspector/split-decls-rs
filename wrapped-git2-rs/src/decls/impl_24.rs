macro_rules! impl_24 {
    () => {
        impl RepositoryOpenFlags { is_bit_set ! (is_no_search , RepositoryOpenFlags :: NO_SEARCH) ; is_bit_set ! (is_cross_fs , RepositoryOpenFlags :: CROSS_FS) ; is_bit_set ! (is_bare , RepositoryOpenFlags :: BARE) ; is_bit_set ! (is_no_dotgit , RepositoryOpenFlags :: NO_DOTGIT) ; is_bit_set ! (is_from_env , RepositoryOpenFlags :: FROM_ENV) ; }
    };
}

impl_24!()