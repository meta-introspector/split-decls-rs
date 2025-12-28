macro_rules! deps {
    () => {
        Mutation!();
        Object!();
        Context!();
        Request!();
    };
}

macro_rules! Upload {
    () => {
        deps!();
        # [doc = " Uploaded file"] # [doc = ""] # [doc = " **Reference:** <https://github.com/jaydenseric/graphql-multipart-request-spec>"] # [doc = ""] # [doc = ""] # [doc = " Graphql supports file uploads via `multipart/form-data`."] # [doc = " Enable this feature by accepting an argument of type `Upload` (single file)"] # [doc = " or `Vec<Upload>` (multiple files) in your mutation like in the example blow."] # [doc = ""] # [doc = ""] # [doc = " # Example"] # [doc = " *[Full Example](<https://github.com/async-graphql/examples/blob/master/models/files/src/lib.rs>)*"] # [doc = ""] # [doc = " ```"] # [doc = " use async_graphql::*;"] # [doc = ""] # [doc = " struct Mutation;"] # [doc = ""] # [doc = " #[Object]"] # [doc = " impl Mutation {"] # [doc = "     async fn upload(&self, ctx: &Context<'_>, file: Upload) -> bool {"] # [doc = "         println!(\"upload: filename={}\", file.value(ctx).unwrap().filename);"] # [doc = "         true"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " # Example Curl Request"] # [doc = ""] # [doc = " Assuming you have defined your Mutation like in the example above,"] # [doc = " you can now upload a file `myFile.txt` with the below curl command:"] # [doc = ""] # [doc = " ```curl"] # [doc = " curl 'localhost:8000' \\"] # [doc = " --form 'operations={"] # [doc = "         \"query\": \"mutation ($file: Upload!) { upload(file: $file)  }\","] # [doc = "         \"variables\": { \"file\": null }}' \\"] # [doc = " --form 'map={ \"0\": [\"variables.file\"] }' \\"] # [doc = " --form '0=@myFile.txt'"] # [doc = " ```"] # [derive (Debug , Clone , Copy , Ord , PartialOrd , Eq , PartialEq , Hash)] pub struct Upload (pub usize) ;
    };
}

Upload!();