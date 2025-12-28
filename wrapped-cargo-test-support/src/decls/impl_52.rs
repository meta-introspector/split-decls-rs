macro_rules! deps {
    () => {
        RepoBuilder!();
        SymlinkBuilder!();
        Repository!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl RepoBuilder { pub fn init (p : & Path) -> RepoBuilder { t ! (fs :: create_dir_all (p . parent () . unwrap ())) ; let repo = init (p) ; RepoBuilder { repo , files : Vec :: new () , } } # [doc = " Add a file to the repository."] pub fn file (self , path : & str , contents : & str) -> RepoBuilder { let mut me = self . nocommit_file (path , contents) ; me . files . push (PathBuf :: from (path)) ; me } # [doc = " Create a symlink to a directory"] pub fn nocommit_symlink_dir < T : AsRef < Path > > (self , dst : T , src : T) -> Self { let workdir = self . repo . workdir () . unwrap () ; SymlinkBuilder :: new_dir (workdir . join (dst) , workdir . join (src)) . mk () ; self } # [doc = " Add a file that will be left in the working directory, but not added"] # [doc = " to the repository."] pub fn nocommit_file (self , path : & str , contents : & str) -> RepoBuilder { let dst = self . repo . workdir () . unwrap () . join (path) ; t ! (fs :: create_dir_all (dst . parent () . unwrap ())) ; t ! (fs :: write (& dst , contents)) ; self } # [doc = " Create the repository and commit the new files."] pub fn build (self) -> Repository { { let mut index = t ! (self . repo . index ()) ; for file in self . files . iter () { t ! (index . add_path (file)) ; } t ! (index . write ()) ; let id = t ! (index . write_tree ()) ; let tree = t ! (self . repo . find_tree (id)) ; let sig = t ! (self . repo . signature ()) ; t ! (self . repo . commit (Some ("HEAD") , & sig , & sig , "Initial commit" , & tree , & [])) ; } let RepoBuilder { repo , .. } = self ; Repository (repo) } }
    };
}

impl_52!()