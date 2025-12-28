macro_rules! InMemoryDir {
    () => {
        pub struct InMemoryDir { files : Vec < (PathBuf , Data) > , }
    };
}

InMemoryDir!()